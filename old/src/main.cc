#include <iostream>
#include <fstream>
#include <string>

#include <gflags/gflags.h>

#include "engine.h"

// TODO: Consider an engine configuration struct.
DEFINE_bool(dump_module, false, "Dump the generated module to stdout.");
DEFINE_bool(optimize, true, "Optimize the generated code.");

int main(int argc, char* argv[]) {
  std::string usage("Start a REPL or run a script.\n");
  usage += std::string("    ") + argv[0] + "[script]\n";
  google::SetUsageMessage(usage);
  google::ParseCommandLineFlags(&argc, &argv, true);

  std::string file = argc > 1 ? argv[1] : std::string();
  engine::Initialize(file, FLAGS_optimize);
  return engine::Run(FLAGS_dump_module) ? 0 : 1;
}
