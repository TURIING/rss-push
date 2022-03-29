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
        
        HStack {
            GeometryReader { geoProxy in
                VStack{
                    if let data = self.contentData.subscribed {
                        Spacer()
                        ScrollView {
                            ScrollViewReader { scroProxy in
                                LazyVGrid(columns: column, alignment: .leading, spacing: 8) {
                                    ForEach(data, id: \.self) { item in
                                        
                                        SubscribedListItem(resCrateInfo: item, messageNum: 3)
                                            .id(item.crateId)
                                            .onTapGesture {
                                                self.contentData.selectedSubscribedItem = item.crateId
                                                self.contentData.selectedSubscribedItemPosition = item.crateId
                                            }
                                    }
                                }
                                .onAppear{
                                    scroProxy.scrollTo(self.contentData.selectedSubscribedItemPosition)
                                }
                            }
                        }
                        .frame(height: geoProxy.size.height - 55)
                        
                    } else {
                        Text("no subscribed")
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
