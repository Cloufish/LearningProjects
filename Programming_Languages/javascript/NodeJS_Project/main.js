var server = require('http');

server.createServer(function(request, response){
    
    var url = request.url;
    response.writeHead(200, {'Content-Type': 'text/plain'})

    console.log(url);
    response.end(url);
}).listen(8001);

console.log('Server nasluchuje pod adresem http://127.0.0.1:8001');