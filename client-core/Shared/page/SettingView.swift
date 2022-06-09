//
//  SettingView.swift
//  client-core
//
//  Created by turiing on 2022/3/23.
//

import SwiftUI

struct SettingView: View {
    @EnvironmentObject var contentData: ContentViewModel
    @State private var checked = true
    var body: some View {
        Spacer()
        HStack(alignment: .top) {
            VStack(alignment: .leading) {
                
                Text("常规")
                    .padding(.top, 50)
                Toggle(isOn: $contentData.isReceive) {
                    Text("接收消息通知")
                }.toggleStyle(CheckboxToggleStyle())
                
                
                Spacer()
            }
            
        }
        .padding(.trailing, 100)
        Spacer()
    }
}

struct SettingView_Previews: PreviewProvider {
    static var previews: some View {
        SettingView()
    }
}
