//
//  constant.swift
//  client-core
//
//  Created by turiing on 2022/3/24.
//
import SwiftUI

let serverHost: URL = URL(string: "http://127.0.0.1:8000")!
let loginUrlPath: String = "api/auth/login"
let registerUrlPath: String = "api/auth/register"
let rssInfoUrlPath: String = "api/rss/info"
let rssSubscribeUrlPath: String = "api/rss/subscribe"
let rssUnsubscribeUrlPath: String = "api/rss/unsubscribe"
let subscribedUrlPath: String = "api/crate/subscribed"
let refreshMessageUrlPath: String = "api/crate/refresh"
let readMessageUrlPath: String = "api/crate/read"
let unreadMessageUrlPath: String = "api/crate/unread"
