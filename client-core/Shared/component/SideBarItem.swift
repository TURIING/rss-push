//
//  SideBarItem.swift
//  client-core
//
//  Created by turiing on 2022/3/22.
//

import SwiftUI

struct SideBarItem: View {
    @State var name: String = ""
    @State var image: String = ""
    @EnvironmentObject var contentData: ContentViewModel
    
    var body: some View {
        
        Button(action: {withAnimation{contentData.selectTab = name}}) {
            VStack(spacing: 7) {
                Image(systemName: image)
                    .font(.system(size: 16))
                    .foregroundColor(contentData.selectTab == name ? Color.theme.foreground : .gray)
                Text(name)
                    .font(.system(size: 11))
                    .foregroundColor(contentData.selectTab == name ? Color.theme.foreground : .gray)
            }
        }
        .buttonStyle(PlainButtonStyle())
        .frame(width: 70, height: 55)
        .background(Color.theme.sideBarItemBg.opacity(contentData.selectTab == name ? 0.35:0))
        .cornerRadius(10)
        .animation(.easeOut)
        
        
    }
}


