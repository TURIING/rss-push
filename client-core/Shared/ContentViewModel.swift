//
//  ContentViewModel.swift
//  client-core
//
//  Created by turiing on 2022/3/23.
//

import SwiftUI
import Foundation
import UserNotifications

class ContentViewModel: ObservableObject {
    @Published var selectTab: String = "Message"
    @Published var rssError: RssError? = nil
    @Published var subscribed: [ResCrateInfo]? = nil
    @Published var messages: [ResMessagesInfo]? = nil
    @Published var selectedSubscribedItem: String? = nil
    @Published var selectedSubscribedItemPosition: String? = nil
    @Published var selectedMessageItem: String? = nil
    @Published var selectedMessageItemPosition: String? = nil
    @Published var selectedMessageContent: String? = nil
    @Published var isReceive: Bool = true
    @Published var isShowAccountView: Bool = !Account.instance.info.isLogin
    private var webSocketTask: URLSessionWebSocketTask?
    
    func connect(username: String) {
        
        let url = URL(string: "ws:127.0.0.1:9002")!
        webSocketTask = URLSession.shared.webSocketTask(with: url)
        webSocketTask?.receive(completionHandler: onReceive)
        webSocketTask?.resume()
        webSocketTask?.send(.string(username)) { error in
            if let error = error {
                print("Error sending message", error)
            }
        }
    }
    
    private func onReceive(incoming: Result<URLSessionWebSocketTask.Message, Error>) {
        webSocketTask?.receive(completionHandler: onReceive)
        
        if case .success(let message) = incoming {
            if case .string(let text) = message {
                guard let data = text.data(using: .utf8),
                      let info = try? JSONDecoder().decode([ResMessagesInfo].self, from: data)
                else {
                    return
                }
                print(info)
                DispatchQueue.main.async {
                    if(!NSApplication.shared.isActive && self.self.isReceive) {
                        print("no active")
                        // 请求通知权限
                        UNUserNotificationCenter.current().requestAuthorization(options: [.alert, .badge, .sound]) { success,error in
                            if success {
                                print("request authorization")
                            } else if let error = error {
                                print(error.localizedDescription)
                            }
                            
                        }
                        // 发送通知
                        let content = UNMutableNotificationContent()
                        content.title = info[0].title
                        content.subtitle = info[0].description
                        content.sound = UNNotificationSound.default
                        
                        let trigger = UNTimeIntervalNotificationTrigger(timeInterval: 0.1, repeats: false)
                        let request = UNNotificationRequest(identifier: UUID().uuidString, content: content, trigger: trigger)
                        UNUserNotificationCenter.current().add(request)
                    }
                    if self.self.messages == nil {
                        self.self.messages = []
                    }
                    self.messages!.append(contentsOf: info)
                    
                }
            }
        } else if case .failure(let err) = incoming {
            print(err)
        }
    }
}
