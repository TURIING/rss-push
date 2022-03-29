//
//  other.swift
//  client-core
//
//  Created by turiing on 2022/3/22.
//
import SwiftUI
import WebKit
import Alamofire

// hide textfile ring focus
extension NSTextField {
    open override var focusRingType: NSFocusRingType {
        get {.none}
        set{}
    }
}

// make view blur
struct BlurView: NSViewRepresentable {
    func makeNSView(context: Context) -> NSVisualEffectView {
        let view = NSVisualEffectView()
        view.blendingMode = .behindWindow
        
        return view
    }
    
    func updateNSView(_ nsView: NSViewType, context: Context) {
        
    }
}

// wkwebview
#if os(iOS)
struct HTMLStringView: UIViewRepresentable {
    let htmlContent: String

    func makeUIView(context: Context) -> WKWebView {
        return WKWebView()
    }

    func updateUIView(_ uiView: WKWebView, context: Context) {
        uiView.loadHTMLString(htmlContent, baseURL: nil)
    }
}
#else
struct HTMLStringView: NSViewRepresentable {
    let htmlContent: String

    func makeNSView(context: Context) -> WKWebView {
        return WKWebView()
    }

    func updateNSView(_ nsView: WKWebView, context: Context) {
        nsView.loadHTMLString(htmlContent, baseURL: nil)
    }
}
#endif


// custom color set
extension Color {
    static let theme = ColorTheme()
    
}
struct ColorTheme {
    let foreground = Color("Foreground")
    let background = Color("Background")
    let sideBarItemBg = Color("SideBarItemBg")
    let sideBarItemFg = Color("SideBarItemFg")
    let sheetBg = Color("SheetBg")
    let accountViewTextFieldBg = Color("AccountViewTextFieldBg")
}

// modify TextField style for AccountView
struct CustomTextFieldStyleView<Content: View>: View {
    let content: Content
    
    init(@ViewBuilder content: () -> Content) {
        self.content = content()
    }
    
    var body: some View {
        content
            .textFieldStyle(PlainTextFieldStyle())
            .font(.system(size: 14))
            .lineLimit(1)
            .disableAutocorrection(true)
            .padding(.horizontal, 15)
            .padding(.vertical, 10)
    }
}







