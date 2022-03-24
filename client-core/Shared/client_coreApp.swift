//
//  client_coreApp.swift
//  Shared
//
//  Created by turiing on 2022/3/21.
//

import SwiftUI

@main
struct client_coreApp: App {
    @StateObject var contentData: ContentViewModel = ContentViewModel()
    var body: some Scene {
        WindowGroup {
            ContentView()
                #if os(macOS)
                .frame(idealWidth: 1200, maxWidth: .infinity, idealHeight: 600, maxHeight: .infinity, alignment: .leading)
                #endif                
                .environmentObject(contentData)
                
                
        }
        #if os(macOS)
        .windowStyle(HiddenTitleBarWindowStyle())
        #endif
    }
}
