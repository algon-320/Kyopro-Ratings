# Kyopro-Ratings

競プロサイトのレーティングをJSON形式で返すHTTPサーバです。

## リクエスト・レスポンス

以下の形式でGETリクエストを送ると結果のJSONを返します。
```
/json?atcoder=AtCoderのユーザ名&codeforces=Codeforcesのユーザ名&topcoder_algorithm=TopCoderのユーザ名&topcoder_marathon=TopCoderのユーザ名
```

クエリは`{コンテストサイト名}={ユーザ名}`の形式になっています。
対応しているコンテストサイト名は以下の通りです。
|クエリ中のキー|説明|
|-:|:-|
|`atcoder`|AtCoderレーティング(アルゴリズム)|
|`codeforces`|Codeforcesレーティング|
|`topcoder_algorithm`|TopCoderレーティング(SRM)|
|`topcoder_marathon`|TopCoderレーティング(マラソンマッチ)|

レスポンスの例:
```
{
  "atcoder": {
    "color": "#0000FF",
    "rating": 1735,
    "status": "success"
  },
  "codeforces": {
    "color": "#0000FF",
    "rating": 1801,
    "status": "success"
  },
  "topcoder_algorithm": {
    "color": "#DDCC00",
    "rating": 1502,
    "status": "success"
  },
  "topcoder_marathon": {
    "color": "#00A900",
    "rating": 1003,
    "status": "success"
  }
}
```


## 公開APIサーバ

[@su8ru](https://su8ru.dev/)さんにAPIサーバをホストしていただいています。
こちらは以下のURLで利用可能です。
```
http://kyopro-ratings.jp1.su8.run/json
```
