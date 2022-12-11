// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use crate::teaclave_management_service_proto as proto;

pub use proto::TeaclaveManagement;
pub use proto::TeaclaveManagementClient;
pub use proto::TeaclaveManagementRequest;
pub use proto::TeaclaveManagementResponse;

pub type RegisterInputFileRequest = crate::teaclave_frontend_service::RegisterInputFileRequest;
pub type UpdateInputFileRequest = crate::teaclave_frontend_service::UpdateInputFileRequest;
pub type RegisterInputFileResponse = crate::teaclave_frontend_service::RegisterInputFileResponse;
pub type UpdateInputFileResponse = crate::teaclave_frontend_service::UpdateInputFileResponse;
pub type RegisterOutputFileRequest = crate::teaclave_frontend_service::RegisterOutputFileRequest;
pub type UpdateOutputFileRequest = crate::teaclave_frontend_service::UpdateOutputFileRequest;
pub type RegisterOutputFileResponse = crate::teaclave_frontend_service::RegisterOutputFileResponse;
pub type UpdateOutputFileResponse = crate::teaclave_frontend_service::UpdateOutputFileResponse;
pub type RegisterFusionOutputRequest =
    crate::teaclave_frontend_service::RegisterFusionOutputRequest;
pub type RegisterFusionOutputResponse =
    crate::teaclave_frontend_service::RegisterFusionOutputResponse;
pub type RegisterInputFromOutputRequest =
    crate::teaclave_frontend_service::RegisterInputFromOutputRequest;
pub type RegisterInputFromOutputResponse =
    crate::teaclave_frontend_service::RegisterInputFromOutputResponse;
pub type GetInputFileRequest = crate::teaclave_frontend_service::GetInputFileRequest;
pub type GetInputFileResponse = crate::teaclave_frontend_service::GetInputFileResponse;
pub type GetOutputFileRequest = crate::teaclave_frontend_service::GetOutputFileRequest;
pub type GetOutputFileResponse = crate::teaclave_frontend_service::GetOutputFileResponse;
pub type RegisterFunctionRequest = crate::teaclave_frontend_service::RegisterFunctionRequest;
pub type RegisterFunctionRequestBuilder =
    crate::teaclave_frontend_service::RegisterFunctionRequestBuilder;
pub type RegisterFunctionResponse = crate::teaclave_frontend_service::RegisterFunctionResponse;
pub type UpdateFunctionRequest = crate::teaclave_frontend_service::UpdateFunctionRequest;
pub type UpdateFunctionRequestBuilder =
    crate::teaclave_frontend_service::UpdateFunctionRequestBuilder;
pub type UpdateFunctionResponse = crate::teaclave_frontend_service::UpdateFunctionResponse;
pub type DeleteFunctionRequest = crate::teaclave_frontend_service::DeleteFunctionRequest;
pub type DeleteFunctionResponse = crate::teaclave_frontend_service::DeleteFunctionResponse;
pub type DisableFunctionRequest = crate::teaclave_frontend_service::DisableFunctionRequest;
pub type DisableFunctionResponse = crate::teaclave_frontend_service::DisableFunctionResponse;
pub type GetFunctionRequest = crate::teaclave_frontend_service::GetFunctionRequest;
pub type GetFunctionResponse = crate::teaclave_frontend_service::GetFunctionResponse;
pub type ListFunctionsRequest = crate::teaclave_frontend_service::ListFunctionsRequest;
pub type ListFunctionsResponse = crate::teaclave_frontend_service::ListFunctionsResponse;
pub type CreateTaskRequest = crate::teaclave_frontend_service::CreateTaskRequest;
pub type CreateTaskResponse = crate::teaclave_frontend_service::CreateTaskResponse;
pub type GetTaskRequest = crate::teaclave_frontend_service::GetTaskRequest;
pub type GetTaskResponse = crate::teaclave_frontend_service::GetTaskResponse;
pub type AssignDataRequest = crate::teaclave_frontend_service::AssignDataRequest;
pub type AssignDataResponse = crate::teaclave_frontend_service::AssignDataResponse;
pub type ApproveTaskRequest = crate::teaclave_frontend_service::ApproveTaskRequest;
pub type ApproveTaskResponse = crate::teaclave_frontend_service::ApproveTaskResponse;
pub type InvokeTaskRequest = crate::teaclave_frontend_service::InvokeTaskRequest;
pub type InvokeTaskResponse = crate::teaclave_frontend_service::InvokeTaskResponse;
pub type CancelTaskRequest = crate::teaclave_frontend_service::CancelTaskRequest;
pub type CancelTaskResponse = crate::teaclave_frontend_service::CancelTaskResponse;
