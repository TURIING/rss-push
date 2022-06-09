//
//  SideBarItemModel.swift
//  client-core
//
//  Created by turiing on 2022/3/23.
//

import SwiftUI

let sideBarItemData: [SideBarItemType] = [
    //SideBarItemType(name: "Home", image: "house"),
    SideBarItemType(name: "Message", image: "bubble.left"),
    SideBarItemType(name: "Subscribe", image: "link"),
    SideBarItemType(name: "Setting", image: "gearshape")
]

struct SideBarItemType: Hashable {
    var name: String
    var image: String
}
