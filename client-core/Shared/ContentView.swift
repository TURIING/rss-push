//
//  ContentView.swift
//  Shared
//
//  Created by turiing on 2022/3/21.
//

import SwiftUI


struct ContentView: View {
    @EnvironmentObject var contentData: ContentViewModel
    @StateObject var account = Account.instance
    @State var isShowSubscribeRssView: Bool = false
    
    
    
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
                case "Subscribe": SubscribedView();
                case "Setting": SettingView();
                default : Text("None")
                }
            }
            .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .leading)
        }
        .onAppear{
            if account.isLogin() {
                contentData.connect(username: account.info.username!)
            }
        }
        .onChange(of: contentData.isShowAccountView) { _ in
            if !contentData.isShowAccountView {
                contentData.connect(username: account.info.username!)
            }
            
        }
        .ignoresSafeArea(.all, edges: .all)
        #if os(macOS)
        .toolbar {
            ToolbarItem(placement: .status) {
                Button(action: {isShowSubscribeRssView.toggle()}) {
                    Image(systemName: "plus")
                        .foregroundColor(Color.theme.foreground)
                }
                .padding(.leading, 250)
                .sheet(isPresented: $isShowSubscribeRssView) {
                    SubscribeRssView(isShowSubscribeRssView: $isShowSubscribeRssView)
                }
            }
        }
        #endif
        .alert(item: $contentData.rssError) { rssError in
            Alert(title: Text("Error"), message: Text(rssError.error.localizedDescription))
        }
        .sheet(isPresented: $contentData.isShowAccountView) {
            AccountView()
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
