//
//  AccountView.swift
//  client-core
//
//  Created by turiing on 2022/3/22.
//

import SwiftUI

struct AccountView: View {
    @Binding var isShowAccountView: Bool
    @State private var username: String = ""
    @State private var passwd: String = ""
    @State private var passwdAgain: String = ""
    @State private var disable: Bool = true
    @EnvironmentObject var contentData: ContentViewModel
    // Text for show some information
    @State private var alertText: String = ""
    // toggle login or register view
    @State private var isToggleView: Bool = false
    
    var body: some View {
        ZStack {
            Color.theme.sheetBg
            
            if self.isToggleView {
                registerView
            } else {
                loginView
            }
            
        }
        .frame(width: 300, height: 400)
    }
    
    var loginView: some View {
        VStack {
            VStack(spacing: 5) {
                CustomTextFieldStyleView {
                    TextField("Username", text: $username)
                        .onChange(of: username) { _ in
                            if !passwd.isEmpty && !username.isEmpty {
                                self.disable = false
                            }else {
                                self.disable = true
                            }
                        }
                    SecureField("Password", text: $passwd)
                        .onChange(of: passwd) { _ in
                            if !passwd.isEmpty && !username.isEmpty {
                                self.disable = false
                            }else {
                                self.disable = true
                            }
                        }
                }
            }
            .background(Color.theme.accountViewTextFieldBg.opacity(0.5))
            .cornerRadius(10)
            .frame(width: 240, height: 100)
                                
            Button(action: {
                AccountViewModel.instance.login(username: username, passwd: passwd) { result in
                    switch result {
                    case .success(_):
                        isShowAccountView.toggle()
                        #if os(macOS)
                        NSApp.mainWindow?.endSheet(NSApp.keyWindow!)
                        #endif
                    case .failure(let err):
                        switch err {
                        case AccountError.passwdMistake: self.alertText = "The entered password is incorrect"
                        case AccountError.userNotExist: self.alertText = "The user does not exist"
                        case AccountError.alreadyLogged: self.alertText = "You have logged in to the account"
                        case RssErrorType.jsonDecodeError: self.alertText = "Json decode error"
                        case RssErrorType.networkError: self.alertText = "Network error"
                        default: self.alertText = "Unknown error"
                        }
                    }
                }
            }) {
                Text("Login")
            }
            .frame(width: 240, height: 30)
            .disabled(disable)
            .buttonStyle(PlainButtonStyle())
            .background(Color.theme.accountViewTextFieldBg)
            .cornerRadius(5)
            
            Text(alertText).foregroundColor(.red)
            
            Button(action:{
                self.toggleView()
            }) {
                Text("Go to register").foregroundColor(.blue)
            }
            .buttonStyle(PlainButtonStyle())
            .padding(.leading, 150)
        }
    }
    
    var registerView: some View {
        VStack {
            VStack(spacing: 10) {
                CustomTextFieldStyleView {
                    TextField("Username", text: $username)
                        .onChange(of: username) { _ in
                            if !passwd.isEmpty && !username.isEmpty && !passwdAgain.isEmpty {
                                self.disable = false
                            }else {
                                self.disable = true
                            }
                        }
                    SecureField("Password", text: $passwd)
                        .onChange(of: passwd) { _ in
                            if !passwd.isEmpty && !username.isEmpty && !passwdAgain.isEmpty {
                                if passwd != passwdAgain {
                                    self.alertText = "The entered passwords are inconsistent"
                                } else {
                                    self.alertText = ""
                                }
                                self.disable = false
                            }else {
                                self.disable = true
                            }
                        }
                    SecureField("Password again", text: $passwdAgain)
                        .onChange(of: passwdAgain) { _ in
                            if !passwd.isEmpty && !username.isEmpty && !passwdAgain.isEmpty {
                                if passwd != passwdAgain {
                                    self.alertText = "The entered passwords are inconsistent"
                                } else {
                                    self.alertText = ""
                                }
                                self.disable = false
                            }else {
                                self.disable = true
                            }
                        }
                }
            }
            .background(Color.theme.accountViewTextFieldBg.opacity(0.5))
            .cornerRadius(10)
            .frame(width: 240, height: 100)
                                
            Button(action: {
                AccountViewModel.instance.register(username: username, passwd: passwd) { result in
                    switch result {
                    case .success(_):
                        self.alertText = "Registered successfully"
                    case .failure(let err):
                        switch err {
                        case AccountError.alreadyRegister: self.alertText = "The account has already been registered."
                        default: self.alertText = "Unknown error"
                        }
                    }
                }
            }) {
                Text("Register")
            }
            .frame(width: 240, height: 30)
            .disabled(disable)
            .buttonStyle(PlainButtonStyle())
            .background(Color.theme.accountViewTextFieldBg)
            .cornerRadius(5)
            .padding(.top, 30)
            
            Text(alertText).foregroundColor(.red)
            
            
            
            Button(action:{
                self.toggleView()
            }) {
                Text("back").foregroundColor(.blue)
            }
            .buttonStyle(PlainButtonStyle())
            .padding(.trailing, 200)
        }
    }
    
    private func toggleView() {
        withAnimation(.linear(duration: 0.3)){
            self.isToggleView.toggle()
        }
        self.username = ""
        self.passwd = ""
        self.passwdAgain = ""
        self.alertText = ""
    }
}


