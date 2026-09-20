import SwiftUI

struct CheckoutView: View {
    @EnvironmentObject private var heatmap: HeatmapSession
    @State private var couponCode = ""
    @State private var password = ""

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 24) {
                Text("Checkout")
                    .font(.largeTitle)

                Group {
                    Text("Order summary")
                        .font(.headline)
                    Text("Heatmap Pro — annual subscription")
                    Text("Review details before you pay.")
                }

                Group {
                    Text("Delivery")
                        .font(.headline)
                    Text("Digital delivery — no shipping address required.")
                    Text("Scroll to reach payment and coupon controls.")
                }
                .padding(.top, 400)

                Group {
                    Text("Coupon")
                        .font(.headline)
                    HStack {
                        TextField("Coupon", text: $couponCode)
                            .textFieldStyle(.roundedBorder)
                        Button("Apply coupon") {
                            if HeatmapE2ECaptureHooks.usesWindowTapCapture {
                                heatmap.recordViewportScroll()
                                heatmap.recordIdentifiedActivation(elementId: CheckoutIdentifiers.couponApply)
                                heatmap.flushBatch()
                            }
                        }
                        .accessibilityIdentifier(CheckoutIdentifiers.couponApply)
                    }
                }

                Group {
                    Text("Account")
                        .font(.headline)
                    SecureField("Account password", text: $password)
                        .textFieldStyle(.roundedBorder)
                }

                Group {
                    Text("Payment")
                        .font(.headline)
                    HStack {
                        NavigationLink("Pay") {
                            ConfirmationView()
                        }
                        .accessibilityIdentifier(CheckoutIdentifiers.payButton)
                        .heatmapE2EIdentifiedTap(CheckoutIdentifiers.payButton, heatmap: heatmap)
                        .buttonStyle(.borderedProminent)
                        NavigationLink("Cancel") {
                            ProductView()
                        }
                    }
                }
            }
            .padding()
        }
        .simultaneousGesture(
            DragGesture(minimumDistance: 8).onChanged { _ in
                heatmap.recordViewportScroll()
            }
        )
        .navigationTitle("Checkout")
        .onAppear {
            heatmap.trackScreen("Checkout", layoutVersion: "2024-09")
        }
    }
}
