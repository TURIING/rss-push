//
//  SubscribedListItem.swift
//  client-core
//
//  Created by turiing on 2022/3/28.
//

import SwiftUI

struct SubscribedListItem: View {
    @State var resCrateInfo: ResCrateInfo
    @EnvironmentObject var contentData: ContentViewModel
    @State var messageNum: Int
    
    var body: some View {
        
            
        HStack(spacing: 0) {
            VStack(alignment: .leading, spacing: 8) {
                Text(self.resCrateInfo.crateInfo.title)
                Text(self.resCrateInfo.crateInfo.description)
                    .font(.system(size: 10))
                    .lineLimit(1)
                    .truncationMode(.tail)
                    .foregroundColor(Color.gray)
            }
            .padding(.leading, 10)
            
            Spacer()
            
            VStack(spacing: 8) {
                Text("17:00")
                    .font(.caption2)
                Text(String(messageNum))
                    .font(.caption2)
                    .foregroundColor(Color.white)
                    .padding(4)
                    .background(Color.blue)
                    .clipShape(Circle())
                    .padding(.horizontal, 5)
            }
            .padding(.leading, 30)
            .padding(.trailing, 3)
            
            
                
            
        }
        .frame(width: 295, height: 60)
        .background(Color.blue.opacity(self.contentData.selectedSubscribedItem == self.resCrateInfo.crateId ? 0.2 : 0))
        .clipShape(RoundedRectangle(cornerRadius: 7))
            
            
        
        
        
        
        
    }
}

