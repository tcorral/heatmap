import XCTest
@testable import HeatmapAppleShared

final class HeatmapFlushSchedulerTests: XCTestCase {
    func testPerformDoesNotRunOnMainThread() throws {
        let scheduler = HeatmapFlushScheduler()
        var workThread: Thread?
        try scheduler.perform {
            workThread = Thread.current
        }
        XCTAssertNotNil(workThread)
        XCTAssertFalse(workThread!.isMainThread)
    }

    func testPollingDoesNotRunOnMainThread() {
        let expectation = expectation(description: "poll")
        var pollThread: Thread?
        let scheduler = HeatmapFlushScheduler(pollInterval: 0.05) {
            pollThread = Thread.current
            expectation.fulfill()
        }
        wait(for: [expectation], timeout: 1.0)
        scheduler.stop()
        XCTAssertNotNil(pollThread)
        XCTAssertFalse(pollThread!.isMainThread)
    }

    func testPerformStillRunsWork() throws {
        let scheduler = HeatmapFlushScheduler()
        var ran = false
        try scheduler.perform {
            ran = true
        }
        XCTAssertTrue(ran)
    }
}
