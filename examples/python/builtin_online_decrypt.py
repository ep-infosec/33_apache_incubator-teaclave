#!/usr/bin/env python3

# Licensed to the Apache Software Foundation (ASF) under one
# or more contributor license agreements.  See the NOTICE file
# distributed with this work for additional information
# regarding copyright ownership.  The ASF licenses this file
# to you under the Apache License, Version 2.0 (the
# "License"); you may not use this file except in compliance
# with the License.  You may obtain a copy of the License at
#
#   http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing,
# software distributed under the License is distributed on an
# "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
# KIND, either express or implied.  See the License for the
# specific language governing permissions and limitations
# under the License.

import sys
import base64

from utils import USER_ID, USER_PASSWORD, connect_authentication_service, connect_frontend_service


class BuiltinOnlineDecryptExample:

    def __init__(self, user_id, user_password):
        self.user_id = user_id
        self.user_password = user_password

    def decrypt(self, key, nonce, encrypted_data, algorithm):
        with connect_authentication_service() as client:
            print("[+] login")
            token = client.user_login(self.user_id, self.user_password)

        client = connect_frontend_service()
        metadata = {"id": self.user_id, "token": token}
        client.metadata = metadata

        print("[+] registering function")
        function_id = client.register_function(
            name="builtin_online_decrypt",
            description="Native Online Decrypt",
            executor_type="builtin",
            arguments=["key", "nonce", "encrypted_data", "algorithm"])

        print("[+] creating task")
        task_id = client.create_task(
            function_id=function_id,
            function_arguments={
                "key": dataToBase64(key),
                "nonce": dataToBase64(nonce),
                "encrypted_data":
                "CaZd8qSMMlBp8SjSXj2I4dQIuC9KkZ5DI/ATo1sWJw==",
                "algorithm": algorithm
            },
            executor="builtin")

        print("[+] invoking task")
        client.invoke_task(task_id)

        print("[+] getting result")
        result = client.get_task_result(task_id)
        print("[+] done")
        client.close()

        return bytes(result)


def dataToBase64(data):
    return base64.standard_b64encode(bytes(data)).decode("utf-8")


def main():
    example = BuiltinOnlineDecryptExample(USER_ID, USER_PASSWORD)
    key = [
        106, 165, 29, 129, 157, 37, 38, 123, 179, 247, 40, 143, 146, 128, 241,
        51, 166, 92, 77, 197, 85, 165, 222, 10, 40, 186, 179, 108, 112, 252,
        240, 184
    ]
    nonce = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
    encrypted_data = "CaZd8qSMMlBp8SjSXj2I4dQIuC9KkZ5DI/ATo1sWJw=="
    algorithm = "aes256gcm"  # Alogorithm can be "aes256gcm" or "aes128gcm"
    rt = example.decrypt(key, nonce, encrypted_data, algorithm)
    print("[+] function return: ", rt)


if __name__ == '__main__':
    main()
