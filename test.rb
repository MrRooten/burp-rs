client = RBHttpClient.new
request = {
    "method" => "get",
    "url" => "https://cn.bing.com"
}
print("hello: ")
puts client.send(request)