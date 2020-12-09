import json

type
    Message* = object
        username*: string
        message*: string
proc parseMessage*(data: string): Message =
    let dataJSON = parseJson(data)
    result.username = dataJson["username"].getStr()
    result.message = dataJson["message"].getStr()

block:
    let data = """{"username": "John", "message": "Hi!""""
    try:
        let parsed = parseMessage(data)
        doAssert false
    except JsonParsingError:
       doAssert true
    except:
        doAssert false


