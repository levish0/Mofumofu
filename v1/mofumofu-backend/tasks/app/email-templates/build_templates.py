from pathlib import Path
from mjml import mjml_to_html

def build_mjml_templates():
    """MJML 템플릿들을 HTML로 변환"""
    
    # 디렉토리 경로 설정
    current_dir = Path(__file__).parent
    src_dir = current_dir / "src"
    build_dir = current_dir / "build"
    
    # build 디렉토리 생성
    build_dir.mkdir(exist_ok=True)
    
    # src 디렉토리의 모든 .mjml 파일 처리
    mjml_files = list(src_dir.glob("*.mjml"))
    
    if not mjml_files:
        print("No MJML files found.")
        return
    
    print(f"Processing {len(mjml_files)} MJML files...")
    
    for mjml_file in mjml_files:
        print(f"Processing: {mjml_file.name}")
        
        try:
            # MJML 파일 읽기
            with open(mjml_file, 'r', encoding='utf-8') as f:
                mjml_content = f.read()
            
            # MJML을 HTML로 변환
            result = mjml_to_html(mjml_content)
            html_content = result.html
            
            # HTML 파일로 저장
            html_filename = mjml_file.stem + ".html"
            html_path = build_dir / html_filename
            
            with open(html_path, 'w', encoding='utf-8') as f:
                f.write(html_content)
            
            print(f"+ {html_filename} created")
            
        except Exception as e:
            print(f"- Failed to process {mjml_file.name}: {e}")
    
    print("Template build complete!")

if __name__ == "__main__":
    build_mjml_templates()