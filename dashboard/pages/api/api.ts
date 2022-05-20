/*
 * Copyright 2022 Singularity Data
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */
const baseUrl = "http://127.0.0.1:5691/";

class Api {
  baseUrl: string;

  constructor(url: string) {
    this.baseUrl = url;
  }

  async get(path: string) {
    const url = `${this.baseUrl}${path}`;
    try {
      const response = await fetch(url);
      if (response.ok) {
        return response;
      }
    } catch (e) {
      if (e instanceof Error) {
        throw new Error(`${e.message}: ${url}`);
      }
    }
  }
}

export default new Api(baseUrl);