#include <Windows.h>
#include <iostream>

#pragma comment(lib, "version.lib")

void GetFileVersion(char const *pFilePath) {
  DWORD dwSize = 0;
  BYTE *pVersionInfo = NULL;
  VS_FIXEDFILEINFO *pFileInfo = NULL;
  UINT pLenFileInfo = 0;

  /*getting the file version info size */
  dwSize = GetFileVersionInfoSize(pFilePath, NULL);
  if (dwSize == 0) {
    printf("Error in GetFileVersionInfoSize: %lu\n", GetLastError());
    return;
  }

  /*allocation of space for the verison size */
  pVersionInfo = new BYTE[dwSize];
  std::cout << "size: " << dwSize << '\n';

  /*entering all info data to pbVersionInfo*/
  if (!GetFileVersionInfo(pFilePath, 0, dwSize, pVersionInfo)) {
    printf("Error in GetFileVersionInfo: %lu\n", GetLastError());
    delete[] pVersionInfo;
    return;
  }

  if (!VerQueryValueA(pVersionInfo, TEXT("\\"), (LPVOID *)&pFileInfo,
                      &pLenFileInfo)) {
    printf("Error in VerQueryValue: %lu\n", GetLastError());
    delete[] pVersionInfo;
    return;
  }

  std::cout << std::hex << (pFileInfo->dwFileVersionMS >> 16) << '\n';
  std::cout << std::hex << (pFileInfo->dwFileVersionLS >> 16) << '\n';
  std::cout << ((pFileInfo->dwFileVersionMS >> 16) & 0xffff) << '\n';
  std::cout << ((pFileInfo->dwFileVersionMS) & 0xffff) << '\n';
  std::cout << ((pFileInfo->dwFileVersionLS >> 16) & 0xffff) << '\n';
  std::cout << ((pFileInfo->dwFileVersionLS) & 0xffff) << '\n';
}

int main(int argc, char *argv[]) {
  char const *path =
      "D:/Release/AsrRMALog/AsrRMALog(v2.0.4)/AsrRMALogGenerator.exe";
  GetFileVersion(path);
}
