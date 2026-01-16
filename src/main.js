const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;
const { openUrl } = window.__TAURI__.opener;

let appCountEl;
let countBtn;
let analyzeBtn;
let githubBtn;
let bgm;
let muteBtn;
let isMuted = false;
let isMusicPlaying = false;
let isAnalyzeEnabled = false; // 标记分析按钮是否可用
let cleanupDetectionStarted = null; // 保存事件监听清理函数

async function searchinstalled() {
  try {
    // 禁用统计按钮，防止重复点击
    countBtn.disabled = true;
    countBtn.textContent = "统计中...";
    
    const count = await invoke("search_installed");
    appCountEl.textContent = `已发现 ${count} 个应用`;
    
    // 添加成功状态样式
    appCountEl.classList.add('ready');
    
    // 解锁分析按钮
    unlockAnalyzeButton();
    
  } catch (error) {
    console.error("调用失败:", error);
    appCountEl.textContent = "统计失败，请重试";
    appCountEl.classList.remove('ready');
  } finally {
    // 重新启用统计按钮
    countBtn.disabled = false;
    countBtn.textContent = "统计已安装的应用";
  }
}

function unlockAnalyzeButton() {
  isAnalyzeEnabled = true;
  analyzeBtn.disabled = false;
  analyzeBtn.title = "点击开始分析已安装的应用";
  
  // 添加解锁动画效果
  analyzeBtn.style.animation = 'pulse 0.5s';
  setTimeout(() => {
    analyzeBtn.style.animation = '';
  }, 500);
}

// 创建浏览器信息卡片
function createBrowserCard(browserInfo) {
  const card = document.createElement('div');
  card.className = 'browser-card';
  
  const icon = document.createElement('img');
  icon.className = 'browser-icon';
  icon.src = browserInfo.icon;
  icon.alt = `${browserInfo.displayName} icon`;
  
  const name = document.createElement('h3');
  name.className = 'browser-name';
  name.textContent = browserInfo.displayName;
  
  const type = document.createElement('p');
  type.className = 'browser-type';
  type.textContent = `类型: ${browserInfo.browserType}`;
  
  const size = document.createElement('p');
  size.className = 'browser-size';
  size.textContent = `大小: ${formatSize(browserInfo.size)}`;
  
  card.appendChild(icon);
  card.appendChild(name);
  card.appendChild(type);
  card.appendChild(size);
  
  return card;
}

function formatSize(bytes) {
  if (bytes === 0) return '0 B';
  
  const units = ['B', 'KiB', 'MiB', 'GiB', 'TiB'];
  const i = Math.floor(Math.log(bytes) / Math.log(1024));
  const value = bytes / Math.pow(1024, i);
  
  return `${value.toFixed(2)} ${units[i]}`;
}

// 设置事件监听
function setupDetectionListener() {
  // 先清理之前的监听器
  if (cleanupDetectionStarted) {
    cleanupDetectionStarted();
  }
  
  // 设置新的监听器
  listen('detection-started', (event) => {
    console.log('收到检测开始事件:', event.payload);
    
    // 创建并添加卡片到网格
    const gridContainer = document.getElementById('browsers-grid');
    if (gridContainer) {
      const card = createBrowserCard(event.payload);
      gridContainer.appendChild(card);
    }
  }).then((cleanup) => {
    cleanupDetectionStarted = cleanup;
  }).catch(error => {
    console.error('设置事件监听失败:', error);
  });
}

// 清空容器内容（保留静音按钮和GitHub按钮）
function clearContainer() {
  const container = document.querySelector('.container');
  const muteBtn = document.getElementById('mute-btn');
  const githubBtn = document.getElementById('github-btn');
  const bgmElement = document.getElementById('bgm');
  
  // 保存静音按钮、GitHub按钮和背景音乐
  const elementsToKeep = [];
  if (muteBtn) elementsToKeep.push(muteBtn);
  if (githubBtn) elementsToKeep.push(githubBtn);
  if (bgmElement) elementsToKeep.push(bgmElement);
  
  // 清空容器
  container.innerHTML = '';
  
  // 重新添加保留的元素
  elementsToKeep.forEach(element => {
    container.appendChild(element);
  });
  
  // 设置容器背景为透明
  container.style.backgroundColor = 'transparent';
  container.style.backdropFilter = 'none';
  container.style.boxShadow = 'none';
  container.style.borderRadius = '0';
  
  // 设置body背景为全透明
  document.body.style.backgroundImage = 'none';
  document.body.style.backgroundColor = 'transparent';
}

// 打开GitHub页面
async function openGitHub() {
  await openUrl('https://github.com/netphils/cefdetector-standalone');
}

async function analyzeApps() {
  if (!isAnalyzeEnabled) {
    alert("请先统计已安装的应用");
    return;
  }
  
  try {
    // 清空容器并设置透明背景
    clearContainer();
    
    // 播放背景音乐
    if (!isMusicPlaying) {
      playBGM();
      isMusicPlaying = true;
    }
    
    // 创建网格容器
    const gridContainer = document.createElement('div');
    gridContainer.id = 'browsers-grid';
    gridContainer.className = 'browsers-grid';
    
    // 创建标题
    const title = document.createElement('h1');
    title.className = 'analysis-title';
    title.textContent = '浏览器分析结果';
    
    // 创建网格说明
    const description = document.createElement('p');
    description.className = 'grid-description';
    description.textContent = '正在检测浏览器...';
    
    // 添加到容器
    const container = document.querySelector('.container');
    container.appendChild(document.createElement('br'));
    container.appendChild(document.createElement('br'));
    container.appendChild(document.createElement('br'));
    container.appendChild(document.createElement('br'));
    container.appendChild(document.createElement('br'));
    container.appendChild(document.createElement('br'));
    container.appendChild(title);
    container.appendChild(description);
    container.appendChild(gridContainer);
    
    // 设置事件监听
    setupDetectionListener();
    
    // 调用Rust函数
    console.log("开始分析浏览器...");
    const browser_summary = await invoke("search_browsers");
    const browser_count = browser_summary.count;
    const browser_size = browser_summary.size;
    console.log(`发现 ${browser_count} 个浏览器，总大小${formatSize(browser_size)}。（大小包含整个程序）`);
    
    // 更新描述
    if (browser_count > 0) {
      description.textContent = `已发现 ${browser_count} 个浏览器。详细信息：`;
    } else {
      description.textContent = '未发现浏览器';
    }
    
  } catch (error) {
    console.error("分析失败:", error);
    
    // 显示错误信息
    const container = document.querySelector('.container');
    const errorMsg = document.createElement('div');
    errorMsg.className = 'error-message';
    errorMsg.textContent = '分析过程中发生错误，请重试';
    errorMsg.style.color = '#ff4444';
    errorMsg.style.padding = '20px';
    errorMsg.style.textAlign = 'center';
    
    const gridContainer = document.getElementById('browsers-grid');
    if (gridContainer) {
      gridContainer.appendChild(errorMsg);
    } else {
      container.appendChild(errorMsg);
    }
  }
}

function playBGM() {
  if (bgm) {
    // 如果已经静音，就取消静音后再播放
    if (isMuted) {
      bgm.muted = false;
      muteBtn.classList.remove('active');
      isMuted = false;
    }
    
    bgm.volume = 0.5; // 设置初始音量
    bgm.play().then(() => {
      console.log("背景音乐开始播放");
    }).catch(error => {
      console.error("音乐播放失败:", error);
      // 有些浏览器不让自动播放
    });
  }
}

function toggleMute() {
  if (!bgm) return;
  
  isMuted = !isMuted;
  bgm.muted = isMuted;
  
  if (isMuted) {
    muteBtn.classList.add('active');
    console.log("背景音乐已静音");
  } else {
    muteBtn.classList.remove('active');
    console.log("背景音乐取消静音");
  }
}

window.addEventListener("DOMContentLoaded", () => {
  appCountEl = document.querySelector("#app-count");
  countBtn = document.querySelector("#count-btn");
  analyzeBtn = document.querySelector("#analyze-btn");
  githubBtn = document.querySelector("#github-btn");
  bgm = document.querySelector("#bgm");
  muteBtn = document.querySelector("#mute-btn");
  
  // 初始状态：分析按钮禁用
  analyzeBtn.disabled = true;
  analyzeBtn.title = "未统计";
  
  // 绑定点击事件
  countBtn.addEventListener("click", (e) => {
    e.preventDefault();
    searchinstalled();
  });
  
  analyzeBtn.addEventListener("click", (e) => {
    e.preventDefault();
    analyzeApps();
  });
  
  // 绑定GitHub按钮事件
  if (githubBtn) {
    githubBtn.addEventListener("click", (e) => {
      e.preventDefault();
      openGitHub();
    });
  }
  
  // 绑定静音按钮事件
  if (muteBtn) {
    muteBtn.addEventListener("click", toggleMute);
  }
  
  // 监听音频事件
  if (bgm) {
    bgm.addEventListener('error', (e) => {
      console.error("音频加载错误:", e);
    });
    
    bgm.addEventListener('play', () => {
      console.log("音乐开始播放");
    });
    
    bgm.addEventListener('pause', () => {
      console.log("音乐暂停");
    });
    
    bgm.addEventListener('ended', () => {
      console.log("音乐播放结束，将重新开始循环播放");
    });
  }
  
  // 预加载音频
  if (bgm) {
    bgm.load();
  }
  
  // 添加CSS动画
  const style = document.createElement('style');
  style.textContent = `
    @keyframes pulse {
      0% { transform: scale(1); }
      50% { transform: scale(1.05); }
      100% { transform: scale(1); }
    }
  `;
  document.head.appendChild(style);
});