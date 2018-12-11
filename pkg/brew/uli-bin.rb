class ULI < Formula
  desc "Universal Loan Identifier (ULI) validation and check digit generation for the Home Mortgage Disclosure Act (HMDA)"
  version '0.2.0'
  url ""
  sha256 ""
  head "https://github.com/jmarin/uli.git"
  depends_on "rust" => :build

  def install
    system "cargo", "install", "--root", prefix
  end

  test do 
    system "#{bin}/uli"
  end 
end

