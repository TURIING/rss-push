//
//  SubscribeView.swift
//  client-core
//
//  Created by turiing on 2022/3/23.
//

import SwiftUI

struct SubscribedView: View {
    @EnvironmentObject var contentData: ContentViewModel
    let column: [GridItem] = [GridItem(.fixed(100))]
    
    var body: some View {
        GeometryReader { geoProxy in
        
            HStack(alignment: .center) {
                VStack(alignment: .center){
                    if let data = self.contentData.subscribed {
                        Spacer()
                        ScrollView {
                            ScrollViewReader { scroProxy in
                                LazyVGrid(columns: column, alignment: .leading, spacing: 8) {
                                    ForEach(data, id: \.self) { item in
                                        
                                        SubscribedListItem(resCrateInfo: item, messageNum: 11)
                                            .id(item.crateId)
                                            .onTapGesture {
                                                self.contentData.selectedSubscribedItem = item.crateId
                                                self.contentData.selectedSubscribedItemPosition = item.crateId
                                            }
                                            .contextMenu{
                                                Button(action:{
                                                    Rss.unsubscribe(crate_id: item.crateId){ result in
                                                        switch result {
                                                        case .success(_):
                                                            let index = self.contentData.subscribed?.firstIndex(of: item)
                                                            self.contentData.subscribed?.remove(at: index!)
                                                        case .failure(let err):
                                                            self.contentData.rssError = RssError(error: err as! RssErrorType)
                                                            return
                                                        }
                                                    }
                                                }) {
                                                    Text("unsubscribe")
                                                }
                                            }
                                    }
                                }
                                .onAppear{
                                    scroProxy.scrollTo(self.contentData.selectedSubscribedItemPosition)
                                }
                            }
                        }
                        .frame(height: geoProxy.size.height - 55)
                        
                    }
                }
                .onAppear{
                    // Gets the subscribed list.
                    Crate.Subscribed(){ result in
                        switch result {
                        case .success(let suc):
                            if let data = suc {
                                self.contentData.subscribed = data
                            } else {
                                self.contentData.subscribed = nil
                            }
                        case .failure(let err):
                            self.contentData.rssError = RssError(error: err as! RssErrorType)
                            return
                        }
                    }
                }
            }
            
            
        }
        .background(BlurView())
        .frame(width: 300)
        
        
        
    }
}

struct SubscribeView_Previews: PreviewProvider {
    static var previews: some View {
        SubscribedView()
    }
}
