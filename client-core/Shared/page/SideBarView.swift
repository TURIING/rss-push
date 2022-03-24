//
//  SideBarView.swift
//  client-core
//
//  Created by turiing on 2022/3/22.
//

import SwiftUI

struct SideBarView: View {
    var body: some View {
        VStack(spacing: 20) {
            ForEach(sideBarItemData, id: \.self) {item in
                if item.name != "Setting" {
                    SideBarItem(name: item.name, image: item.image)
                }

            }
            Spacer()
            Avatar()
            SideBarItem(name: sideBarItemData[3].name, image: sideBarItemData[3].image)
        }
        
        
    }
}

struct SideBarView_Previews: PreviewProvider {
    static var previews: some View {
        SideBarView()
    }
}
