class ULI < Formula
  desc "Universal Loan Identifier (ULI) validation and check digit generation for the Home Mortgage Disclosure Act (HMDA)"
  version 'v0.2.0'
  url "https://github.com/jmarin/uli/archive/#{version}.tar.gz"
  sha256 "51bd372617fdc159639181b508e04fd7aaea244e"
  head "https://github.com/jmarin/uli.git"
  depends_on "rust" => :build

  def install
    system "cargo", "install", "--root", prefix
  end

  test do 
    system "#{bin}/uli"
  end 
end

