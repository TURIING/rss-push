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
    var body: some View {
        ZStack {
            Color.theme.sheetBg
            VStack(alignment: .center, spacing: 80) {
                HStack {
                    CustomTextFieldStyleView {
                        TextField("Feed url", text: $rssUrl)
                            .onChange(of: rssUrl) { _ in
                                if self.rssUrl.isEmpty {
                                    self.isDisableSearchButton = true
                                } else {
                                    self.isDisableSearchButton = false
                                }
                            }
                    }
                    .background(Color.theme.accountViewTextFieldBg.opacity(0.5))
                    .cornerRadius(7)
                    .frame(width: 400)
                    
                    Button(action:{
//                        searchRss(url: rssUrl){ result in
//                            switch result {
//                            case .success(let info):
//                                self.alertText = info.description
//                            case .failure(let err):
//                                switch err {
//                                case SubscribeError.urlInvalid:
//                                    alertText = "This url is invalid"
//                                default:
//                                    alertText = err.localizedDescription
//                                }
//                            }
//                        }
                    }) {
                        Image(systemName: "magnifyingglass")
                    }
                    .buttonStyle(PlainButtonStyle())
                    .disabled(isDisableSearchButton)
                    .padding(.leading, 15)
                }
                
                Text(alertText).foregroundColor(.red)
                
                //Spacer()
                Button(action: {
                    isShowSubscribeRssView.toggle()
                    #if os(macOS)
                    NSApp.mainWindow?.endSheet(NSApp.keyWindow!)
                    #endif
                }) {
                    Text("close")
                }
            }
        }
        .frame(width: 500, height: 400)
    }
    
//    private func searchRss(url: String, completion: @escaping (Result<RssInfo, Error>) -> Void) {
//        guard let feedUrl = URL(string: url) else {
//            completion(.failure(SubscribeError.urlInvalid))
//            return
//        }
//
//        let url = serverHost.appendingPathComponent(rssInfoUrlPath)
//        let param = ["url", feedUrl.absoluteString]
//
//        AF.request(
//            url,
//            method: .post,
//            parameters: param
//        ).responseData { res in
//            switch res.result {
//            case let .success(data):
//                if let data = try? JSONDecoder().decode(RssInfo, from: data){
//
//                }else {
//
//                }
//
//            case .failure(_):
//                completion(.failure(RssErrorType.networkError))
//            }
//        }
//
//    }
}

struct RssInfo: Codable {
    var title: String
    var description: String
}

