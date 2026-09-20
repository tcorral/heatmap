import Foundation
import XCTest

struct HeatmapE2EEventClient {
    func fetchEvents() async throws -> [[String: Any]] {
        let url = HeatmapE2EConfig.apiBaseURL.appendingPathComponent("v1/events")
        let (data, response) = try await URLSession.shared.data(from: url)
        guard let http = response as? HTTPURLResponse, http.statusCode == 200 else {
            throw URLError(.badServerResponse)
        }
        let json = try JSONSerialization.jsonObject(with: data) as? [String: Any]
        return json?["events"] as? [[String: Any]] ?? []
    }

    func latestSessionEvents(from all: [[String: Any]]) -> [[String: Any]] {
        guard
            let lastStart = all.last(where: { HeatmapE2EEventAssertions.eventType($0) == "session_start" }),
            let sessionId = HeatmapE2EEventAssertions.sessionId(lastStart)
        else {
            return all
        }
        return all.filter { HeatmapE2EEventAssertions.sessionId($0) == sessionId }
    }

    func waitForEvents(
        minimumCount: Int,
        timeout: TimeInterval = 45,
        pollInterval: TimeInterval = 0.5
    ) async throws -> [[String: Any]] {
        let deadline = Date().addingTimeInterval(timeout)
        var lastCount = 0
        while Date() < deadline {
            let allEvents = try await fetchEvents()
            let events = latestSessionEvents(from: allEvents)
            lastCount = events.count
            if events.count >= minimumCount {
                return events
            }
            try await Task.sleep(nanoseconds: UInt64(pollInterval * 1_000_000_000))
        }
        XCTFail("Timed out waiting for at least \(minimumCount) events in latest session (last count: \(lastCount))")
        return []
    }
}

enum HeatmapE2EEventAssertions {
    static func eventType(_ event: [String: Any]) -> String? {
        (event["event"] as? [String: Any])?["type"] as? String
    }

    static func elementId(_ event: [String: Any]) -> String? {
        (event["event"] as? [String: Any])?["element_id"] as? String
    }

    static func screenId(_ event: [String: Any]) -> String? {
        (event["screen"] as? [String: Any])?["id"] as? String
    }

    static func sessionId(_ event: [String: Any]) -> String? {
        event["session_id"] as? String
    }

    static func clickPosition(_ event: [String: Any]) -> (x: Double, y: Double)? {
        guard eventType(event) == "click",
              let position = event["position"] as? [String: Any],
              let x = position["x"] as? Double,
              let y = position["y"] as? Double
        else {
            return nil
        }
        return (x, y)
    }

    static func assertSharedCheckoutSequence(_ events: [[String: Any]], file: StaticString = #filePath, line: UInt = #line) {
        let types = events.compactMap(eventType)
        XCTAssertTrue(types.contains("session_start"), file: file, line: line)
        XCTAssertTrue(types.contains("scroll"), file: file, line: line)
        XCTAssertGreaterThanOrEqual(types.filter { $0 == "click" }.count, 3, file: file, line: line)

        let screenViews = events
            .filter { eventType($0) == "screen_view" }
            .compactMap { screenId($0)?.lowercased() }
        for expected in ["home", "checkout", "confirmation"] {
            XCTAssertTrue(screenViews.contains(expected), "Missing screen_view \(expected)", file: file, line: line)
        }

        let clickIds = events.filter { eventType($0) == "click" }.compactMap(elementId)
        for expected in ["product", "checkout.coupon", "checkout.pay"] {
            XCTAssertTrue(clickIds.contains(expected), "Missing click \(expected)", file: file, line: line)
        }
    }

    static func assertNormalizedClickCoordinates(_ events: [[String: Any]], file: StaticString = #filePath, line: UInt = #line) {
        let positions = events.compactMap(clickPosition)
        XCTAssertFalse(positions.isEmpty, "Expected at least one click with position", file: file, line: line)
        for position in positions {
            XCTAssertGreaterThanOrEqual(position.x, 0, file: file, line: line)
            XCTAssertLessThanOrEqual(position.x, 1, file: file, line: line)
            XCTAssertGreaterThanOrEqual(position.y, 0, file: file, line: line)
            XCTAssertLessThanOrEqual(position.y, 1, file: file, line: line)
        }
    }

    static func assertSingleSession(_ events: [[String: Any]], file: StaticString = #filePath, line: UInt = #line) {
        let sessionIds = Set(events.compactMap(sessionId))
        XCTAssertEqual(sessionIds.count, 1, "Expected one session_id across events, got \(sessionIds)", file: file, line: line)
    }
}
