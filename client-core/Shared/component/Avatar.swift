//
//  Avatar.swift
//  client-core
//
//  Created by turiing on 2022/3/23.
//

import SwiftUI

struct Avatar: View {
    @EnvironmentObject var contentData: ContentViewModel
    var body: some View {
        Button(action: {}) {
            Image(systemName: "person.crop.circle")
                .clipShape(Circle())
                .font(.system(size: 18))
                .foregroundColor(Color.theme.foreground)
        }
        .buttonStyle(PlainButtonStyle())
        .contextMenu{
            VStack {
                Button(action:{
                    Account.instance.logout()
                    contentData.isShowAccountView.toggle()
                    contentData.messages = nil
                    contentData.subscribed = nil
                    contentData.selectedMessageContent = nil
                }) {
                    Text("logout")
                }
            }
        }
        
        
    }
}

struct Avatar_Previews: PreviewProvider {
    static var previews: some View {
        Avatar()
    }
}
