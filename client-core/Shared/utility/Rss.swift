//
//  Rss.swift
//  client-core
//
//  Created by turiing on 2022/3/26.
//

import SwiftUI
import Alamofire

class Rss {
    static func searchRss(url: String, completion: @escaping (Result<RssInfo, Error>) -> Void) {
        guard let feedUrl = URL(string: url) else {
            completion(.failure(RssErrorType.rssInvalid))
            return
        }

        let url = serverHost.appendingPathComponent(rssInfoUrlPath)
        let param: [String:[String]] = ["url":[feedUrl.absoluteString]]
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
                    case 203:
                        completion(.success(RssInfo(title: data.title!, description: data.description!)))
                    case 503:
                        completion(.failure(RssErrorType.rssParseError))
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
    
    static func subscribe(url: String, completion: @escaping (Result<(), Error>) -> Void) {
        guard let feedUrl = URL(string: url) else {
            completion(.failure(RssErrorType.rssInvalid))
            return
        }
        
        let url = serverHost.appendingPathComponent(rssSubscribeUrlPath)
        let param: [String:[String]] = ["url":[feedUrl.absoluteString]]
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
                    case 202:
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


struct RssInfo: Codable {
    var title: String
    var description: String
}
