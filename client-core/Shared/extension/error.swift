//
//  error.swift
//  client-core
//
//  Created by turiing on 2022/3/24.
//

import SwiftUI

// custom error enum
enum RssErrorType: Error, LocalizedError {
    // Some error type for login or register operation
    case passwdMistake
    case userNotExist
    case alreadyLogged
    case alreadyRegister
    case invalidToken
    // Some error type for json encode or decode
    case jsonDecodeError
    case jsonEncodeError
    // Some error type for rss subscribe or search
    case rssParseError
    case rssInvalid
    
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
        case .passwdMistake:
            return NSLocalizedString("Password mistake", comment: "")
        case .userNotExist:
            return NSLocalizedString("The user does not exist", comment: "")
        case .alreadyLogged:
            return NSLocalizedString("You have logged in to the account", comment: "")
        case .alreadyRegister:
            return NSLocalizedString("The account has already been registered.", comment: "")
        case .invalidToken:
            return NSLocalizedString("Invalid token", comment: "")
        case .rssParseError:
            return NSLocalizedString("An error occurs when parsing the feed.", comment: "")
        case .rssInvalid:
            return NSLocalizedString("This feed's url is invalid.", comment: "")
        }
    }
}
struct RssError: Identifiable {
    var id = UUID()
    var error: RssErrorType
}
