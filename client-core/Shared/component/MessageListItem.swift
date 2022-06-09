//
//  MessageListItem.swift
//  client-core
//
//  Created by turiing on 2022/4/3.
//

import SwiftUI
import Foundation

struct MessageListItem: View {
    @State var resMessageInfo: ResMessagesInfo
    @EnvironmentObject var contentData: ContentViewModel
    @State var date: String = "date"
    
    
    var body: some View {
        
            
        HStack(spacing: 0) {
            VStack(alignment: .leading, spacing: 8) {
                Text(self.resMessageInfo.title)
                Text(self.resMessageInfo.description)
                    .font(.system(size: 10))
                    .lineLimit(1)
                    .truncationMode(.tail)
                    .foregroundColor(Color.gray)
            }
            .padding(.leading, 10)
            
            Spacer()
            
            VStack(spacing: 5) {
                Text("\(self.date)")
                    .font(.caption2)
                    .padding(.top, 10)
                Color.blue
                    .clipShape(Circle())
                    .frame(width: 8)
                    .padding(.horizontal, 5)
                    .opacity(resMessageInfo.check_status ? 0: 1)
            }
            .padding(.leading, 30)
            .padding(.trailing, 3)
            
            
                
            
        }
        .frame(width: 295, height: 60)
        .background(Color.blue.opacity(self.contentData.selectedMessageItemPosition == self.resMessageInfo.message_id ? 0.2 : 0))
        .clipShape(RoundedRectangle(cornerRadius: 7))
        .onAppear{
            let formater = DateFormatter()
            formater.dateFormat = "E, dd MMM yyyy HH:mm:ss z"
            let date = formater.date(from: self.resMessageInfo.send_time)
            let component = Calendar.current.dateComponents([.month, .day], from: date!)
            self.date = "\(component.month!)/\(component.day!)"
        }
            
        
        
        
        
        
    }
}

