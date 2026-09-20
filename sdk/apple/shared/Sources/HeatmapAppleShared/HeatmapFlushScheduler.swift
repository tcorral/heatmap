import Foundation

/// Serializes native flush/poll off the capture/UI thread.
final class HeatmapFlushScheduler: @unchecked Sendable {
    private let queue = DispatchQueue(label: "com.heatmap.flush")
    private var timer: DispatchSourceTimer?

    init() {}

    init(pollInterval: TimeInterval, poll: @escaping () -> Void) {
        let timer = DispatchSource.makeTimerSource(queue: queue)
        timer.schedule(deadline: .now() + pollInterval, repeating: pollInterval)
        timer.setEventHandler(handler: poll)
        timer.resume()
        self.timer = timer
    }

    func perform(_ work: @escaping () throws -> Void) throws {
        let box = ErrorBox()
        let done = DispatchSemaphore(value: 0)
        queue.async {
            do {
                try work()
            } catch {
                box.error = error
            }
            done.signal()
        }
        done.wait()
        if let error = box.error {
            throw error
        }
    }

    func stop() {
        timer?.cancel()
        timer = nil
    }

    deinit {
        stop()
    }
}

private final class ErrorBox: @unchecked Sendable {
    var error: Error?
}
