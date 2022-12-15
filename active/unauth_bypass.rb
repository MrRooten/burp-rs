class RBModule_unauth_bypass
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
        info = { "name" => "unauth_bypass", 
            "description" => "" }
        return info
    end

    def get_notfound_page(url) 
        resp = Request.get(url)
        return resp.body
    end

    def check(method, url, headers, body, not_found)
        if not_found == nil 
            return 
        end
        resp = Request.get(url, headers, body)
        
        if resp == nil 
            debug("#{url} response is nil")
            return 
        end
        if resp.status != 403 or resp.status != 404 
            if Similary.match(resp.body, not_found) < 0.9 
                host = UriParser.parse(url)['host']
                if host == nil 
                    host = url
                end
                issue = {
                    "name"=> "403 bypass",
                    "level" => "critical",
                    "confidence" => "suspicious",
                    "detail" => "payload #{url} can be used to bypass 403",
                    "host" => host,
                    "response" => resp.orig_resp
                }

                IssueReporter.add_issue(issue)
            end
        end
    end

    def scan(method, uri, headers, body)
        headers.delete_if {|key,value| (key.downcase.contains("cookie") or key.downcase.contains("token"))}
        scheme = uri["scheme"]
        if scheme == nil 
            error("Scheme is none #{uri}")
            return 
        end
        #simeple test
        path = uri["path"]
        notfound_url = scheme + "://" + uri["host"] + ":" + uri["port"].to_s + "/sdklfjsklcbnskdjfsdf"
        notfound = get_notfound_page(notfound_url)
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
                
                check(method, url, headers, body, notfound)
            end
            nodes[i] = node
            i += 1
        end
        
    end

    def passive_run(index)
        log = HistoryLog.get_req index
        url = log.url
        uri = UriParser.parse(url)
        info("Test url: #{uri}")
        scan(log.method, uri, log.headers, log.body)
        #scan("http://127.0.0.1:8009")
    end

end