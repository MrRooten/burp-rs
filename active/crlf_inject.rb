class RBModule_crlf_inject
    def initialize
    end
    def metadata
        info = { "name" => "crlf_inject", 
            "description" => "Test for crlf inject" }
        return info
    end

    def match_headers?(value, headers)
    end
    def scan(uri, headers)
        query = uri.query
        if query == nil 
            return
        end
        
        kvs = query.split("&")
        items = []
        kvs.each do |_kv|
            

        end
        scheme = uri.scheme
        if scheme == nil 
            error("Scheme is none #{uri}")
            return 
        end
        #simeple test
        url = scheme + "://" + uri.host + uri.path + "%0D%0ASet-Cookie:testkey=testvalue"
        client = Request.new
        resp = client.get(url, headers={"host"=>uri.host})
        headers = resp.headers
        if headers == nil 
            return 
        end
        #puts resp
        headers.each do |key,value|
            if key.casecmp("set-cookie")
                if value.contains("testkey")
                    info("Found cookie injection in #{url} #{"vuln".red}")
                    issue = {
                        "name"=> "crlf simple test",
                        "level" => "highrisk",
                        "confidence" => "confirm",
                        "detail" => "this is a detail",
                        "host" => uri.host,
                        "response" => resp
                    }

                    IssueReporter.push_issue(issue)
                end
            end
        end

        
    end

    def passive_run(index)
        log = HistoryLog.get_req index
        resp = HistoryLog.get_resp index
        headers = nil
        if resp == nil 
            headers = nil
        else
            headers = resp['headers']
        end
        url = log['url']
        uri = UriParser.parse(url)
        info("Test url: #{uri}")
        scan(uri, headers)
        #scan("http://127.0.0.1:8009")
    end
    def active_run(url, opts)
    end
end