//
//  ContentViewModel.swift
//  client-core
//
//  Created by turiing on 2022/3/23.
//

import SwiftUI

class ContentViewModel: ObservableObject {
    @Published var selectTab: String = "Home"
    @Published var rssError: RssError? = nil
}
