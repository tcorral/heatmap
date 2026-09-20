import XCTest
@testable import HeatmapAppleShared

final class HeatmapFlushSchedulerTests: XCTestCase {
    func testPerformDoesNotRunOnMainThread() {
        let scheduler = HeatmapFlushScheduler()
        let expectation = expectation(description: "perform")
        var workThread: Thread?
        scheduler.perform {
            workThread = Thread.current
            expectation.fulfill()
        }
        wait(for: [expectation], timeout: 1.0)
        scheduler.stop()
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

    func testPerformStillRunsWork() {
        let scheduler = HeatmapFlushScheduler()
        let expectation = expectation(description: "ran")
        scheduler.perform {
            expectation.fulfill()
        }
        wait(for: [expectation], timeout: 1.0)
        scheduler.stop()
    }

    func testPerformReturnsBeforeBlockedWorkCompletes() {
        let scheduler = HeatmapFlushScheduler()
        let started = expectation(description: "started")
        let finished = expectation(description: "finished")
        var callerContinued = false
        scheduler.perform {
            started.fulfill()
            Thread.sleep(forTimeInterval: 0.2)
            XCTAssertTrue(callerContinued)
            finished.fulfill()
        }
        wait(for: [started], timeout: 1.0)
        callerContinued = true
        wait(for: [finished], timeout: 1.0)
        scheduler.stop()
    }
}
