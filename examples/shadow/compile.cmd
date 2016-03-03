set FXC="%DXSDK_DIR%\Utilities\bin\x64\fxc.exe"
mkdir data
%FXC% /T vs_4_0 /E Vertex /Fo data/forward_vs.fx shader/forward.hlsl
%FXC% /T ps_4_0 /E Pixel /Fo data/forward_ps.fx shader/forward.hlsl
%FXC% /T vs_4_0 /E Vertex /Fo data/shadow_vs.fx shader/shadow.hlsl
%FXC% /T ps_4_0 /E Pixel /Fo data/shadow_ps.fx shader/shadow.hlsl
