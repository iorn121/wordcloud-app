import { sums, wakati, main, test } from "../../pkg/wordcloud_app_bg.wasm";
import React, { useEffect, useState } from "react";
import Papa from "papaparse";

export const App = () => {
  const [json, setJson] = useState("");
  const encodings = ["utf-8", "shift_jis"];
  const [encoding, setEncoding] = useState("utf-8");
  async function convert(e: React.ChangeEvent<HTMLInputElement>) {
    if (e.target.files) {
      Papa.parse<any>(e.target.files[0], {
        header: false,
        encoding: encoding,
        skipEmptyLines: true,
        complete(results: any) {
          const csvData = results.data;
          const jsonText =
            csvData.join("") !== "" ? JSON.stringify(csvData) : "";
          setJson(jsonText);
        },
        error() {
          alert("CSVファイルの読み込みに失敗しました。");
        },
      });
    }
    return json;
  }
  const handleOnChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    convert(e).then((json) => {
      console.log(json);
      // const wakatibun = main(1);
      let contributor = {
        value: "hello",
      };
      let ans = test(contributor);
      console.log(ans);
      // console.log(wakatibun);
    });
  };

  const [value, setValue] = useState(0);
  return (
    <div>
      <input
        onChange={(e) => {
          const v = Number(e.target.value);
          !isNaN(v) && setValue(sums(v));
        }}
      />
      <div>{value}</div>
      <form action="">
        <input type="file" accept=".csv" onChange={handleOnChange} />
        {/* <button onClick={handleSubmit}>IMPORT CSV</button> */}
      </form>
      <select onChange={(e) => setEncoding(e.target.value)}>
        {encodings.map((encoding) => (
          <option key={encoding} value={encoding}>
            {encoding}
          </option>
        ))}
      </select>
      <div>{json}</div>
    </div>
  );
};
export default App;
