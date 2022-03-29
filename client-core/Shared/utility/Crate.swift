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
}
