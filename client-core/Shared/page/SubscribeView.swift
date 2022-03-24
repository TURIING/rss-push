//
//  SubscribeView.swift
//  client-core
//
//  Created by turiing on 2022/3/23.
//

import SwiftUI

struct SubscribeView: View {
    var body: some View {
        ZStack {
            BlurView()
                .frame(width: 300)
            HStack {
                Text("Subscribe page")
                    
            }
            
            
        }
        
    }
}

struct SubscribeView_Previews: PreviewProvider {
    static var previews: some View {
        SubscribeView()
    }
}
