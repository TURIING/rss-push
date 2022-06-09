//
//  type.swift
//  client-core
//
//  Created by turiing on 2022/3/27.
//

import SwiftUI

// msg type return from server.
struct ServerMsg: Codable {
    var msg: String?
    var status: Int
    var token: String?
    var rssInfo: RssInfo?
    var cratesInfo: [ResCrateInfo]?
    var messagesInfo: [ResMessagesInfo]?
    
}
// A type for searching rss.
struct RssInfo: Codable {
    var url: String?
    var title: String
    var description: String
}

// A type for having been subscribed crates.
struct CrateInfo: Codable, Equatable, Hashable {
    var url: String?
    var title: String
    var description: String
}

struct ResCrateInfo: Codable, Equatable, Hashable {
    var crateId: String
    var crateInfo: CrateInfo
}

struct ResMessagesInfo: Codable, Equatable, Hashable {
    var title: String
    var description: String
    var content: String
    var message_id: String
    var crate_id: String
    var check_status: Bool
    var send_time: String
}
