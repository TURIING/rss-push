//
//  error.swift
//  client-core
//
//  Created by turiing on 2022/3/24.
//

import SwiftUI

// custom error enum
enum RssErrorType: Error, LocalizedError {
    case jsonDecodeError
    case jsonEncodeError
    case networkError
    case unknownError
    
    var errorDescription: String? {
        switch self {
        case .jsonDecodeError:
            return NSLocalizedString("The JSON parser encountered an error while parsing json data", comment: "")
        case .jsonEncodeError:
            return NSLocalizedString("The JSON parser encountered an error while encoding json data", comment: "")
        case .networkError:
            return NSLocalizedString("Network error, please check the network", comment: "")
        case .unknownError:
            return NSLocalizedString("Unknown error occurs", comment: "")
        }
    }
}
struct RssError: Identifiable {
    var id = UUID()
    var error: RssErrorType
}

enum AccountError: Error {
    case passwdMistake
    case userNotExist
    case alreadyLogged
    case alreadyRegister
    case invalidToken
}
