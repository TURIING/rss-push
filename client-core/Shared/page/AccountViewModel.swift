//
//  AccountViewModel.swift
//  client-core
//
//  Created by turiing on 2022/3/22.
//

import SwiftUI
import Alamofire


class AccountViewModel: ObservableObject {
    @Published var info: AccountInfo
    
    static let instance = AccountViewModel()
    
    init() {
        self.info = AccountConfig.instance.getInfo()
    }
    
    
    func login(username: String, passwd: String, completion: @escaping (Result<(), Error>) -> Void ) {
        let param = ["username": username, "passwd": passwd]
        let url = serverHost.appendingPathComponent(loginUrlPath)
        AF.request(
            url,
            method: .post,
            parameters: param,
            encoder: JSONParameterEncoder.default
        ).responseData { res in
            switch res.result {
            case let .success(data):
                if let data = try? JSONDecoder().decode(Msg.self, from: data) {
                    switch data.status {
                    case 201:
                        AccountConfig.instance.setLogin(username: username, token: data.token!)
                        completion(.success(()))
                    case 404: completion(.failure(AccountError.userNotExist))
                    case 401: completion(.failure(AccountError.passwdMistake))
                    case 402: completion(.failure(AccountError.alreadyLogged))
                    default: completion(.failure(RssErrorType.unknownError))
                    }
                } else {
                    completion(.failure(RssErrorType.jsonDecodeError))
                }

            case .failure(_):
                completion(.failure(RssErrorType.networkError))
            }
        }
    }
    func register(username: String, passwd: String, completion: @escaping(Result<(), Error>) -> Void) {
        let param = ["username": username, "passwd": passwd]
        let url = serverHost.appendingPathComponent(registerUrlPath)
        AF.request(
            url,
            method: .post,
            parameters: param,
            encoder: JSONParameterEncoder.default
        ).responseData { res in
            switch res.result {
            case let .success(data):
                if let data = try? JSONDecoder().decode(Msg.self, from: data) {
                    switch data.status {
                    case 200: completion(.success(()))
                    case 400: completion(.failure(AccountError.alreadyRegister))
                    default: completion(.failure(RssErrorType.unknownError))
                    }
                } else {
                    completion(.failure(RssErrorType.jsonDecodeError))
                }
            case .failure(_):
                completion(.failure(RssErrorType.networkError))
            }
        }
    }
    func isLogin() -> Bool {
        return AccountConfig.instance.isLogin()
    }
}

// Configuration file Operations
class AccountConfig {
    static let instance = AccountConfig()
    private var catalogUrl: URL
    private var fileUrl: URL
    
    
    init() {
        self.catalogUrl = FileManager.default.urls(for: .applicationDirectory, in: .userDomainMask).first!.appendingPathComponent("Account", isDirectory: true)
        self.fileUrl = catalogUrl.appendingPathComponent("config", isDirectory: false)
        
        // detect and create file
        if !FileManager.default.fileExists(atPath: catalogUrl.path) {
            try! FileManager.default.createDirectory(at: catalogUrl, withIntermediateDirectories: true)
            
        }
        if !FileManager.default.fileExists(atPath: fileUrl.path){
            let primaryContent = AccountInfo(isLogin: false, token: nil, username: nil)
            let content = try! JSONEncoder().encode(primaryContent)
            FileManager.default.createFile(atPath: fileUrl.path, contents: content)
            
        }
    }
    
    private func getConfig() -> AccountInfo {
        let fileData = FileManager.default.contents(atPath: fileUrl.path)!
        return try! JSONDecoder().decode(AccountInfo.self, from: fileData)
    }
    
    private func setConfig(config: AccountInfo) {
        try! JSONEncoder().encode(config).write(to: self.fileUrl)
    }
    
    func setLogin(username: String, token: String) {
        let config = AccountInfo(isLogin: true, token: token, username: username)
        self.setConfig(config: config)
    }
    
    func isLogin() -> Bool {
        let info = getConfig()
        return info.isLogin
    }
    func getInfo() -> AccountInfo {
        return getConfig()
    }
    
}

// The content type of the configuration file
struct AccountInfo: Codable {
    var isLogin: Bool
    var token: String?
    var username: String?
}



