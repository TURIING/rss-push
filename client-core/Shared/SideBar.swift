//
//  SideBar.swift
//  RssClient
//
//  Created by turiing on 2022/3/19.
//

import SwiftUI
import WebKit

//
//
//struct SideBar1: View {
//    @State private var isDefaultActive = false
//    @State var isShow: Bool = false
//
//    var body: some View {
//
//        List {
//            #if os(macOS)
//            Spacer()
//            #endif
//
//            Text("wecome").font(.caption2).foregroundColor(.secondary)
//
//            NavigationLink(destination: HomeItemView(), isActive: $isDefaultActive) {
//
//                Label("Home", systemImage: "house")
//                Spacer()
//                Text("10").foregroundColor(Color("SideBarViewItemCout"))
//
//            }
//
//            NavigationLink(destination: Text("other")) {
//
//                Label("Other", systemImage: "house")
//                Spacer()
//                Text("10").foregroundColor(Color("SideBarViewItemCout"))
//
//            }
//
//            Divider()
//
//        }
//        .listStyle(SidebarListStyle())
//        .frame(minWidth: 250, idealWidth: 250, maxWidth: .infinity, maxHeight: .infinity)
//            .toolbar {
//                Spacer()
//                Button(action: {isShow.toggle()}) {
//                    Image(systemName: "plus.magnifyingglass")
//                }
//                .sheet(isPresented: $isShow) {
//                    VStack {
//                        Text("hello world")
//                        Button("close", action: {
//                            isShow.toggle()
//                            #if os(macOS)
//                            NSApp.mainWindow?.endSheet(NSApp.keyWindow!)
//                            #endif
//                        })
//                    }
//                    .frame(width: 100, height: 100)
//                    .cornerRadius(15)
//                }
//
//            }
        //Text("Select a group").frame(maxWidth: .infinity, maxHeight: .infinity)
        
        
//    }
//}






//struct WebView: NSViewRepresentable {
//
//    let view: WKWebView = WKWebView()
//
//    var request: URLRequest {
//        get{
//            let url: URL = URL(string: "https://google.com")!
//            let request: URLRequest = URLRequest(url: url)
//            return request
//        }
//    }
//
//    func makeNSView(context: Context) -> WKWebView {
//        view.load(request)
//        return view
//    }
//
//    func updateNSView(_ view: WKWebView, context: Context) {
//        view.load(request)
//    }
//
//}






