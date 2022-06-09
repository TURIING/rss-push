//
//  Crate.swift
//  client-core
//
//  Created by turiing on 2022/3/28.
//

import SwiftUI
import Alamofire

class Crate {
    static func Subscribed(completion: @escaping (Result<[ResCrateInfo]?, Error>) -> Void) {
                
        let url = serverHost.appendingPathComponent(subscribedUrlPath)
        let token = Account.instance.info.token
        let headers = HTTPHeaders(["Authorization" : "Bearer " + token!])
        
        AF.request(
            url,
            method: .get,
            headers: headers
        ).responseData { res in
            switch res.result {
            case let .success(data):
                if let data = try? JSONDecoder().decode(ServerMsg.self, from: data){
                    switch data.status {
                    case 204:
                        completion(.success(data.cratesInfo!))
                    case 205:
                        completion(.success(nil))
                    default:
                        completion(.failure(RssErrorType.unknownError))
                    }
                }else {
                    completion(.failure(RssErrorType.jsonDecodeError))
                }

            case .failure(_):
                completion(.failure(RssErrorType.networkError))
            }
        }
    }
    
    static func refresh_message(completion: @escaping (Result<[ResMessagesInfo]?, Error>) -> Void) {
        let url = serverHost.appendingPathComponent(refreshMessageUrlPath)
        let token = Account.instance.info.token
        let headers = HTTPHeaders(["Authorization" : "Bearer " + token!])
        
        AF.request(
            url,
            method: .get,
            headers: headers
        ).responseData { res in
            switch res.result {
            case let .success(data):
                if let data = try? JSONDecoder().decode(ServerMsg.self, from: data){
                    switch data.status {
                    case 206:
                        completion(.success(data.messagesInfo!))
                    case 209:
                        completion(.success(nil))
                    default:
                        completion(.failure(RssErrorType.unknownError))
                    }
                }else {
                    completion(.failure(RssErrorType.jsonDecodeError))
                }

            case .failure(_):
                completion(.failure(RssErrorType.networkError))
            }
        }
    }
    
    static func read_message(message_id: String, completion: @escaping (Result<(), Error>) -> Void) {
        
        let url = serverHost.appendingPathComponent(readMessageUrlPath)
        let param: [String:[String]] = ["message_id":[message_id]]
        let token = Account.instance.info.token
        let headers = HTTPHeaders(["Authorization" : "Bearer " + token!])
        let encoder = URLEncodedFormParameterEncoder(encoder: URLEncodedFormEncoder(arrayEncoding: .noBrackets))
        
        AF.request(
            url,
            method: .post,
            parameters: param,
            encoder: encoder,
            headers: headers
            
        ).responseData { res in
            switch res.result {
            case let .success(data):
                if let data = try? JSONDecoder().decode(ServerMsg.self, from: data){
                    switch data.status {
                    case 207:
                        completion(.success(()))
                    default:
                        completion(.failure(RssErrorType.unknownError))
                    }
                }else {
                    completion(.failure(RssErrorType.jsonDecodeError))
                }

            case .failure(_):
                completion(.failure(RssErrorType.networkError))
            }
        }
    }
    static func unread_message(message_id: String, completion: @escaping (Result<(), Error>) -> Void) {
        
        let url = serverHost.appendingPathComponent(unreadMessageUrlPath)
        let param: [String:[String]] = ["message_id":[message_id]]
        let token = Account.instance.info.token
        let headers = HTTPHeaders(["Authorization" : "Bearer " + token!])
        let encoder = URLEncodedFormParameterEncoder(encoder: URLEncodedFormEncoder(arrayEncoding: .noBrackets))
        
        AF.request(
            url,
            method: .post,
            parameters: param,
            encoder: encoder,
            headers: headers
            
        ).responseData { res in
            switch res.result {
            case let .success(data):
                if let data = try? JSONDecoder().decode(ServerMsg.self, from: data){
                    switch data.status {
                    case 208:
                        completion(.success(()))
                    default:
                        completion(.failure(RssErrorType.unknownError))
                    }
                }else {
                    completion(.failure(RssErrorType.jsonDecodeError))
                }

            case .failure(_):
                completion(.failure(RssErrorType.networkError))
            }
        }
    }
}
