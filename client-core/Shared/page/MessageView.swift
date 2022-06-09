//
//  MessageView.swift
//  client-core
//
//  Created by turiing on 2022/3/23.
//

import SwiftUI

class MessageViewModel: ObservableObject {
    @Published var selectedItemContent: String?
}

struct MessageView: View {
    @EnvironmentObject var contentData: ContentViewModel
    let column: [GridItem] = [GridItem(.fixed(100))]
    
    var body: some View {
        GeometryReader { geoProxy in
            HStack(spacing: 0) {
                VStack{
                    if let data = self.contentData.messages {
                        Spacer()
                        ScrollView {
                            ScrollViewReader { scroProxy in
                                LazyVGrid(columns: column, alignment: .leading, spacing: 8) {
                                    ForEach(data.reversed(), id: \.self) { item in                                    MessageListItem(resMessageInfo: item)
                                            .id(item.message_id)
                                            .onTapGesture {
                                                withAnimation{
                                                    self.contentData.selectedMessageItem = item.message_id
                                                    self.contentData.selectedMessageItemPosition = item.message_id
                                                    self.contentData.selectedMessageContent = self.wrap_content(ctx: item.content, title: item.title)
                                                    self.read_and_refresh(message_id: item.message_id)
                                                    
                                                }
                                            }
                                            .contextMenu {
                                                VStack {
                                                    Button(action:{
                                                        self.read_and_refresh(message_id: item.message_id)
                                                    }) {
                                                        Image(systemName: "star")
                                                        Text("read")
                                                    }
                                                    Button(action:{
                                                        self.unread_and_refresh(message_id: item.message_id)
                                                    }) {
                                                        Image(systemName: "star.slash")
                                                        Text("unread")
                                                    }
                                                }
                                            }
                                    }
                                }
                                .onAppear{
                                    scroProxy.scrollTo(self.contentData.selectedMessageItemPosition)
                                }
                            }
                        }
                        .frame(height: geoProxy.size.height - 55)
                        
                    } else {
                        Text("no message")
                        
                    }
                }
                .background(BlurView())
                .frame(width: 300)
                .onAppear{
                    if Account.instance.info.isLogin {
                        self.refresh()
                        return
                    }
                    
                    
                    if self.contentData.messages == nil {
                        self.contentData.selectedMessageContent = nil
                    }
                }
                
                HTMLStringView(htmlContent: contentData.selectedMessageContent)
                    .frame(width: geoProxy.size.width - 300, height: geoProxy.size.height)
                    .animation(.easeIn)
                
            }
        
        }
        
        
    }
    private func read_and_refresh(message_id: String) {
        Crate.read_message(message_id: message_id) {res in
            switch res {
            case .success(_):
                // Gets the messages
                self.refresh()
            case .failure(let err):
                self.contentData.rssError = RssError(error: err as! RssErrorType)
                return
            }
        }
    }
    private func unread_and_refresh(message_id: String) {
        Crate.unread_message(message_id: message_id) {res in
            switch res {
            case .success(_):
                // Gets the messages
                self.refresh()
            case .failure(let err):
                self.contentData.rssError = RssError(error: err as! RssErrorType)
                return
            }
        }
    }
    private func refresh() {
        Crate.refresh_message(){ result in
            switch result {
            case .success(let data):
                self.contentData.messages = data
            case .failure(let err):
                self.contentData.rssError = RssError(error: err as! RssErrorType)
                return
            }
        }
    }
    private func wrap_content(ctx: String, title: String) -> String {
        """
            <!DOCTYPE html>
            <html>
            <head>
            <meta charset="utf-8">
            <style>
                :root {
                  color-scheme: light dark;
                }
                @media (prefers-color-scheme: dark) {
                    * {
                        color: #BEBEBE;
                        font-weight: lighter;
                        line-height:200%;
                    }
                    h1,h2,h3,h4,h5,h6 {
                        color: #DFDFDF;
                    }
                }
                @media (prefers-color-scheme: light) {
                    * {
                        color: #6F6F6F;
                        font-weight: lighter;
                        line-height:200%;
                    }
                    h1,h2,h3,h4,h5,h6 {
                        color: black;
                    }
                }
                .title {
                    text-align:center;
                    padding-top: 50px;
                }
                
                div.content {
                    width: 85%;
                    margin: 0 auto;
                }
                
                img {
                    width: 100%;
                }
            </style>
            </head>
            <body>
                <div class="content">
                    <h2 class="title">\(title)</h2>
                    \(ctx)
                </div>
            </body>
            </html>
        """
    }
}

struct MessageView_Previews: PreviewProvider {
    static var previews: some View {
        MessageView()
    }
}
