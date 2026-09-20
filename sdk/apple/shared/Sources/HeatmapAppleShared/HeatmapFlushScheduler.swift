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

    func perform(_ work: @escaping () throws -> Void) {
        queue.async {
            try? work()
        }
    }

    func drain() {
        queue.sync {}
    }

    func stop() {
        timer?.cancel()
        timer = nil
        drain()
    }

    deinit {
        stop()
    }
}

