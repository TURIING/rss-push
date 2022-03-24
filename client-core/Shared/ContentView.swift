//
//  ContentView.swift
//  Shared
//
//  Created by turiing on 2022/3/21.
//

import SwiftUI


struct ContentView: View {
    @EnvironmentObject var contentData: ContentViewModel
    @State var isShowSubscribe: Bool = false
    @StateObject var accountViewModel = AccountViewModel.instance
    @State var isShow: Bool = !AccountViewModel.instance.info.isLogin
    
    var body: some View {
        // The default spcing of HStack is 10. I've been looking for this bug for a long time...
        HStack(spacing: 0){
            SideBarView()
                .padding(.horizontal, 7)
                .padding(.top, 70)
                .padding(.bottom, 5)
                .background(BlurView())
            HStack {
                switch contentData.selectTab {
                case "Home": HomeView();
                case "Message": MessageView();
                case "Subscribe": SubscribeView();
                case "Setting": SettingView();
                default : Text("None")
                }
            }
            .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .leading)
        }
        .ignoresSafeArea(.all, edges: .all)
        #if os(macOS)
        .toolbar {
            ToolbarItem(placement: .status) {
                Button(action: toggleSidebar) {
                    Image(systemName: "plus")
                        .foregroundColor(Color.theme.foreground)
                }
                .padding(.leading, 250)
            }
        }
        #endif
        .alert(item: $contentData.rssError) { rssError in
            Alert(title: Text("Error"), message: Text(rssError.error.localizedDescription))
        }
        .sheet(isPresented: $isShow) {
            AccountView(isShow: $isShow).environmentObject(contentData)
        }
        
        
    }
    #if os(macOS)
    private func toggleSidebar() {
        NSApp.keyWindow?.firstResponder?.tryToPerform(#selector(NSSplitViewController.toggleSidebar(_:)), with: nil)
    }
    #endif
}
//            SideBar()

//                    #endif
//                    ToolbarItem(placement: .status) {
//
//                        Button(action: {isShowSubscribe.toggle()}) {
//                            Image(systemName: "plus")
//                        }
//                        .sheet(isPresented: $isShowSubscribe) {
//                            VStack {
//                                Text("hello world")
//                                Button("close", action: {
//                                    isShowSubscribe.toggle()
//                                    #if os(macOS)
//                                    NSApp.mainWindow?.endSheet(NSApp.keyWindow!)
//                                    #endif
//                                })
//                            }
//                            .frame(width: 100, height: 100)
//                            .cornerRadius(15)
//                        }
//                    }
//
//                }
//
//            MessageListView()
            
            
                
        
        
    
    


struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}