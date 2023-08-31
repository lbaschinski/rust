{ runCommand, repoSrc, typos }:

# Runs "typos" on the whole repository (except for git-ignored files).

runCommand "check-typos"
{
  nativeBuildInputs = [
    typos
  ];
} ''
  cd ${repoSrc}
  typos --config .typos.toml
  touch $out
''
