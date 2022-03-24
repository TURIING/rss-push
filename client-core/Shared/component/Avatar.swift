//
//  Avatar.swift
//  client-core
//
//  Created by turiing on 2022/3/23.
//

import SwiftUI

struct Avatar: View {
    var body: some View {
        Button(action: {}) {
            Image(systemName: "person.crop.circle")
                .clipShape(Circle())
                .font(.system(size: 18))
                .foregroundColor(Color.theme.foreground)
        }
        .buttonStyle(PlainButtonStyle())
        
        
    }
}

struct Avatar_Previews: PreviewProvider {
    static var previews: some View {
        Avatar()
    }
}
