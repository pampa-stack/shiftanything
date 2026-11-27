// Exercise the production click dispatcher with the native opener intercepted.
// No browser, account or external process is opened by this test.
import {readFileSync} from 'node:fs';
import vm from 'node:vm';
import assert from 'node:assert/strict';
const root = new URL('../apps/shiftanything-tauri-proof/frontend/', import.meta.url);
const expected = new Map([
 ['website','https://promptica.it/'],
 ['linkedin','https://www.linkedin.com/company/promptica/'],
 ['instagram','https://www.instagram.com/promptica_ai'],
 ['reddit','https://www.reddit.com/user/Comfortable_Ice_4281/'],
 ['x','https://x.com/Pampaeyes'],
 ['discord','https://discord.gg/Xrg2kksz2m'],
]);
for (const language of ['it','en']) {
 const html=readFileSync(new URL(language+'.html',root),'utf8');
 const footer=html.match(/<footer class="status">([\s\S]*?)<\/footer>/)[1];
 let click;
 const calls=[];
 const stop=new Error('click listener captured');
 const context={window:{__TAURI__:{core:{invoke:async (...args)=>{calls.push(args);}}},addEventListener(){}}, document:{
  documentElement:{lang:language},
  querySelector:selector=>selector==='.bar .seg'?{children:[]}: {},
  querySelectorAll:()=>[],
  addEventListener(type, listener, capture){
   if(type==='click' && capture) click=listener;
   else if(type==='load') throw stop;
  },
 }};
 try {vm.runInNewContext(readFileSync(new URL('native.js',root),'utf8'),context);} catch(error) {if(error!==stop) throw error;}
 assert.equal(typeof click,'function');
 for(const [id,url] of expected) {
  const anchor=[...footer.matchAll(/<a\s+([^>]+)>/g)].find(match=>match[1].includes(`href="${url}"`));
  assert.ok(anchor,language+' missing '+id);
  assert.match(anchor[1],/aria-label="[^"]+"/);
  assert.match(anchor[1],/title="[^"]+"/);
  const count=calls.length;
  click({target:{closest:()=>({href:url,closest:()=>true})},preventDefault(){},stopImmediatePropagation(){}});
  await Promise.resolve();
  assert.equal(calls.length,count+1);
  assert.equal(calls.at(-1)[0],'open_social_link');
  assert.equal(calls.at(-1)[1].link,id);
 }
 for(const url of ['https://discord.gg/unknown','https://example.com/','file:///tmp/anything']) {
  const count=calls.length;
  click({target:{closest:()=>({href:url,closest:()=>true})},preventDefault(){},stopImmediatePropagation(){}});
  assert.equal(calls.length,count);
 }
 const count=calls.length;
 click({target:{closest:()=>({href:expected.get('discord'),closest:()=>null})},preventDefault(){},stopImmediatePropagation(){}});
 assert.equal(calls.length,count,'links outside footer cannot invoke social opener');
 console.log(`PASS ${language}: six footer URLs dispatch to closed native IDs; unknown and non-footer links rejected`);
}
