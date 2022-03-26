//
//  SubscribeRssView.swift
//  client-core
//
//  Created by turiing on 2022/3/24.
//

import SwiftUI
import Alamofire

// some rss feed for testing
// http://feed.williamlong.info/
// https://feedx.net/rss/shanghaishuping.xml
// http://www.haijiaoshi.com/feed

struct SubscribeRssView: View {
    @Binding var isShowSubscribeRssView: Bool
    @State var rssUrl: String = ""
    @State var isDisableSearchButton: Bool = true
    @State var alertText: String = ""
    @State var rssTitle: String = ""
    @State var rssDescription: String = ""
    @State var isFinishSearch: Bool = false
    @State var lastSearchUrl: String = ""
    
    private func closeSheet() {
        self.isShowSubscribeRssView.toggle()
        #if os(macOS)
        NSApp.mainWindow?.endSheet(NSApp.keyWindow!)
        #endif
    }
    
    var body: some View {
        ZStack {
            Color.theme.sheetBg
            VStack(alignment: .center, spacing: 40) {
                
                // close button
                Button(action: {
                    self.closeSheet()
                }) {
                    Image(systemName: "xmark")
                }
                .buttonStyle(PlainButtonStyle())
                .padding(.trailing, 250)
                .padding(.top, 20)
                
                Text("Subscribe")
                    .font(.system(size: 26, weight: .semibold))
                    .padding(.trailing, 140)
                
                searchView
                
                              
                
                if isFinishSearch {
                    resultShowView
                }
                
                Spacer()
                
            }
        }
        .frame(width:300, height: 350)
    }
    
    var resultShowView: some View {
        HStack {
            VStack(alignment: .leading, spacing:2) {
                Text(rssTitle)
                    .lineLimit(1)
                    .truncationMode(.tail)
                Text(rssDescription)
                    .font(.system(size: 10))
                    .foregroundColor(.gray)
                    .lineLimit(1)
                    .truncationMode(.tail)
            }
            .padding(.vertical, 10)
            .padding(.leading, 10)
            
            Spacer()
            
            Button(action:{
                Rss.subscribe(url: self.lastSearchUrl) { result in
                    switch result {
                    case .success(_):
                        self.closeSheet()
                    case .failure(let err):
                        self.alertText = err.localizedDescription
                    }
                }
            }){
                Image(systemName: "checkmark.icloud")
                    .font(.system(size: 14))
            }
            .buttonStyle(PlainButtonStyle())
            .padding(.trailing, 10)
            
        }
        .frame(width: 270)
        .background(Color.theme.accountViewTextFieldBg.opacity(0.5))
        .cornerRadius(5)
        
    }
    
    var searchView: some View {
        VStack(spacing: 10) {
            
            HStack {
                CustomTextFieldStyleView {
                    TextField("Feed link", text: $rssUrl)
                        .onChange(of: rssUrl) { _ in
                            if self.rssUrl.isEmpty {
                                self.isDisableSearchButton = true
                                self.isFinishSearch = false
                            } else {
                                self.isDisableSearchButton = false
                            }
                        }
                }
                                    
                Button(action:{
                    Rss.searchRss(url: rssUrl){ result in
                        switch result {
                        case .success(let info):
                            self.isFinishSearch = true
                            self.rssTitle = info.title
                            self.rssDescription = info.description
                            self.lastSearchUrl = self.rssUrl
                        case .failure(let err):
                            alertText = err.localizedDescription
                        }
                    }
                }) {
                    Image(systemName: "magnifyingglass")
                        .font(.system(size: 14))
                }
                .buttonStyle(PlainButtonStyle())
                .disabled(isDisableSearchButton)
                .padding(.trailing, 10)
            }
            .background(Color.theme.accountViewTextFieldBg.opacity(0.5))
            .cornerRadius(5)
            .frame(width: 270)
            
            if !alertText.isEmpty {
                Text(alertText)
                    .foregroundColor(.gray)
                    .lineLimit(1)
                    .truncationMode(.tail)
                    .frame(width: 260)
            }
        }
        
    }
    
    
}



